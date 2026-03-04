//! In-memory representation of Aurora cards and models.
mod card;
mod model_args;

pub use card::*;
pub use model_args::*;
use regex::Regex;
use serde_json::Value;
use std::{
	collections::HashMap,
	convert::TryFrom,
	fs::File,
	io::{BufReader, Read},
	path::{Path, PathBuf},
};
use thiserror::Error;

pub const CARD_TYPE_MISSION: &str = "model";

/// Fully materialized Aurora model state.
#[derive(Debug, Clone)]
pub struct Model {
	pub mission_id: String,
	pub mission_name: String,
	pub model_home_path: PathBuf,
	pub cards: HashMap<String, Card>,
	pub adjacency: HashMap<String, Vec<String>>,
}

impl Model {
	pub fn load(path: &PathBuf) -> Result<Vec<Self>, ModelError> {
		if !path.exists() {
			return Err(ModelError::PathNotFound(path.display().to_string()));
		}

		let mut model_path: PathBuf = path.clone();

		if model_path.is_file() {
			model_path = model_path
				.parent()
				.ok_or_else(|| ModelError::ModelPathNotFound(path.display().to_string()))?
				.to_path_buf();
		}

		if model_path.is_dir() {
			let schema_here = model_path.join("Aurora.schema.json");
			if !schema_here.exists() {
				let child_candidate = model_path.join("aurora");
				let child_schema = child_candidate.join("Aurora.schema.json");
				if child_schema.exists() {
					model_path = child_candidate;
				} else if let Some(parent) = model_path.parent() {
					let parent_schema = parent.join("Aurora.schema.json");
					if parent_schema.exists() {
						model_path = parent.to_path_buf();
					} else {
						return Err(ModelError::SchemaNotFound);
					}
				} else {
					return Err(ModelError::SchemaNotFound);
				}
			}
		} else {
			return Err(ModelError::ModelPathNotFound(
				model_path.display().to_string(),
			));
		}

		// Locate and validate schema file
		let schema_path = model_path.join("Aurora.schema.json");
		if !schema_path.exists() {
			return Err(ModelError::SchemaNotFound);
		}
		Self::validate_schema(&schema_path)?;

		// Locate and validate compact schema file
		let compact_schema_path = model_path.join("Aurora.compact.schema.json");
		if !compact_schema_path.exists() {
			return Err(ModelError::CompactSchemaNotFound);
		}
		Self::validate_schema(&compact_schema_path)?;

		let mut models: Vec<Model> = Self::load_models(&model_path)?;
		if models.is_empty() {
			return Err(ModelError::ModelNotFound);
		}

		// Load all the cards for each model
		for model in models.iter_mut() {
			let mission_path = model_path.join(&model.mission_id);
			if !mission_path.exists() {
				return Err(ModelError::ModelPathNotFound(
					mission_path.display().to_string(),
				));
			}
			Self::load_cards(model, &mission_path)?;
		}

		Ok(models)
	}

	pub fn validate_card_vs_schema(&self, card_id: &str) -> Result<bool, ModelError> {
		let schema_path = self.model_home_path.join("Aurora.schema.json");

		let schema_file = File::open(&schema_path)?;
		let schema: Value = serde_json::from_reader(BufReader::new(schema_file))?;

		let card = self
			.cards
			.get(card_id)
			.ok_or_else(|| ModelError::CardNotFound(card_id.to_string()))?;

		let card_path = self
			.model_home_path
			.join(&self.mission_id)
			.join(&card.card_type)
			.join(format!("{}.json", card.id));
		let card_file = File::open(&card_path)?;
		let card_json: Value = serde_json::from_reader(BufReader::new(card_file))?;

		Ok(jsonschema::is_valid(&schema, &card_json))
	}

	fn load_models(path: &PathBuf) -> Result<Vec<Self>, ModelError> {
		let regex_mission_card = Regex::new(r"^MIS-\d{3}-.+\.json$")?;

		let mut models: Vec<Self> = Vec::new();
		for entry in path.read_dir()? {
			let entry_path = entry?.path();

			if entry_path.is_file() {
				let Some(file_name) = entry_path.file_name().and_then(|name| name.to_str()) else {
					continue;
				};

				if regex_mission_card.is_match(file_name) {
					let mission_card = Card::load(&entry_path)?;

					let mut model = Self {
						mission_id: mission_card.id.clone(),
						mission_name: mission_card.name.clone(),
						model_home_path: path.clone(),
						cards: HashMap::new(),
						adjacency: HashMap::new(),
					};
					model.add_card(mission_card);
					models.push(model);
				}
			}
		}

		Ok(models)
	}

	fn load_cards(model: &mut Model, path: &PathBuf) -> Result<(), ModelError> {
		let mut pending_folders: Vec<PathBuf> = Vec::new();

		for entry in path.read_dir()? {
			let entry = entry?;
			if entry.path().is_dir() {
				pending_folders.push(entry.path());
			}
		}

		while let Some(folder) = pending_folders.pop() {
			for entry in folder.read_dir()? {
				let entry_path = entry?.path();
				if entry_path.is_dir() {
					pending_folders.push(entry_path);
				} else if entry_path.is_file() {
					if let Some(ext) = entry_path.extension() {
						if ext.eq("json") {
							let card = Card::load(&entry_path)?;
							model.add_card(card);
						}
					}
				}
			}
		}

		Ok(())
	}

	fn add_card(&mut self, card: Card) {
		self.cards.insert(card.id.clone(), card.clone());
		if let Some(links) = &card.links {
			for link in links {
				self.adjacency
					.entry(card.id.clone())
					.or_insert_with(Vec::new)
					.push(link.target.clone());
			}
		}
	}

	const MAX_BYTES: usize = 1024 * 1024 * 1024; // 1 GiB
	fn validate_schema(path: &Path) -> Result<(), ModelError> {
		let file = File::open(path)?;
		let mut reader = BufReader::new(file);

		let mut buf = Vec::new();
		reader
			.by_ref()
			.take(Self::MAX_BYTES as u64 + 1) // +1 lets us detect oversize without truncating silently
			.read_to_end(&mut buf)?;

		if buf.len() > Self::MAX_BYTES {
			let actual_size = std::fs::metadata(path)?.len();
			let actual_size = usize::try_from(actual_size).unwrap_or(usize::MAX);
			return Err(ModelError::SchemaTooLarge(
				Self::MAX_BYTES,
				actual_size,
				path.display().to_string(),
			));
		}

		let schema_json: Value = serde_json::from_slice(&buf)?;

		if !jsonschema::meta::is_valid(&schema_json) {
			return Err(ModelError::InvalidSchema(path.display().to_string()));
		}

		Ok(())
	}
}

#[derive(Debug, Error)]
pub enum ModelError {
	#[error("No model found at specified path {0}")]
	PathNotFound(String),
	#[error("Schema file not found")]
	SchemaNotFound,
	#[error("Compact schema file not found")]
	CompactSchemaNotFound,
	#[error("Invalid schema: {0}")]
	InvalidSchema(String),
	#[error("Model root file not found")]
	ModelNotFound,
	#[error("Schema file too large, max {0} bytes ({1} bytes): {2}")]
	SchemaTooLarge(usize, usize, String),
	#[error("Io error: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Regex error: {0}")]
	RegexError(#[from] regex::Error),
	#[error("JSON error: {0}")]
	JsonError(#[from] serde_json::Error),
	#[error("Card error: {0}")]
	CardError(#[from] CardError),
	#[error("Card not found: {0}")]
	CardNotFound(String),
	#[error("Cards not found")]
	CardsNotFound,
	#[error("model path not found: {0}")]
	ModelPathNotFound(String),
}
