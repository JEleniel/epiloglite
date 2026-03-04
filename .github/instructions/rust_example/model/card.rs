use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
	#[serde(rename = "$schema")]
	pub schema: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Map<String, Value>>,
	pub audit_trail: AuditTrail,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub card_subtype: Option<String>,
	pub card_type: String,
	pub description: String,
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub links: Option<Vec<Link>>,
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
}

impl Card {
	pub fn load(path: &PathBuf) -> Result<Self, CardError> {
		let regex_card: Regex = Regex::new(r"^[A-Z]{3}-\d{3}.json$")?;

		if let Some(file_name) = path.file_name() {
			if !regex_card.is_match(&file_name.to_str().ok_or(CardError::InvalidCardFileName)?) {}
		}

		let file = std::fs::File::open(path)?;
		let reader = std::io::BufReader::new(file);
		let card: Card = serde_json::from_reader(reader)?;
		Ok(card)
	}

	pub fn get_links(&self) -> Vec<Link> {
		match &self.links {
			Some(links) => links.clone(),
			None => Vec::new(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
	pub relationship: String,
	pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrail {
	pub hash: Option<String>,
	pub history: Vec<HistoryEntry>,
	pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HistoryEvent {
	Created,
	Edited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
	pub editor: String,
	pub event: HistoryEvent,
	pub timestamp: String,
}

#[derive(Debug, Error)]
pub enum CardError {
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error),
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("Regex error: {0}")]
	RegexError(#[from] regex::Error),
	#[error("Invalid card file name")]
	InvalidCardFileName,
}
