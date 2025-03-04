use winnow::{ascii::Caseless, ModalResult, Parser};

pub fn keyword_parser_abort<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("abort").parse_next(input)
}

pub fn keyword_parser_add<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("add").parse_next(input)
}

pub fn keyword_parser_after<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("after").parse_next(input)
}

pub fn keyword_parser_all<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("all").parse_next(input)
}

pub fn keyword_parser_alter<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("alter").parse_next(input)
}

pub fn keyword_parser_analyze<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("analyze").parse_next(input)
}

pub fn keyword_parser_and<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("and").parse_next(input)
}

pub fn keyword_parser_as<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("as").parse_next(input)
}

pub fn keyword_parser_attach<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("attach").parse_next(input)
}

pub fn keyword_parser_before<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("before").parse_next(input)
}

pub fn keyword_parser_begin<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("begin").parse_next(input)
}

pub fn keyword_parser_between<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("between").parse_next(input)
}

pub fn keyword_parser_by<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("by").parse_next(input)
}

pub fn keyword_parser_case<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("case").parse_next(input)
}

pub fn keyword_parser_cast<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("cast").parse_next(input)
}

pub fn keyword_parser_collate<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("collate").parse_next(input)
}

pub fn keyword_parser_column<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("column").parse_next(input)
}

pub fn keyword_parser_commit<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("commit").parse_next(input)
}

pub fn keyword_parser_create<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("create").parse_next(input)
}

pub fn keyword_parser_database<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("database").parse_next(input)
}

pub fn keyword_parser_default<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("default").parse_next(input)
}

pub fn keyword_parser_deferred<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("deferred").parse_next(input)
}

pub fn keyword_parser_delete<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("delete").parse_next(input)
}

pub fn keyword_parser_detach<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("detach").parse_next(input)
}

pub fn keyword_parser_distinct<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("distinct").parse_next(input)
}

pub fn keyword_parser_drop<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("drop").parse_next(input)
}

pub fn keyword_parser_each<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("each").parse_next(input)
}

pub fn keyword_parser_else<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("else").parse_next(input)
}

pub fn keyword_parser_end<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("end").parse_next(input)
}

pub fn keyword_parser_escape<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("escape").parse_next(input)
}

pub fn keyword_parser_exclusive<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("exclusive").parse_next(input)
}

pub fn keyword_parser_exists<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("exists").parse_next(input)
}

pub fn keyword_parser_explain<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("explain").parse_next(input)
}

pub fn keyword_parser_fail<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("fail").parse_next(input)
}

pub fn keyword_parser_for<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("for").parse_next(input)
}

pub fn keyword_parser_from<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("from").parse_next(input)
}

pub fn keyword_parser_glob<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("glob").parse_next(input)
}

pub fn keyword_parser_group<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("group").parse_next(input)
}

pub fn keyword_parser_having<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("having").parse_next(input)
}

pub fn keyword_parser_if<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("if").parse_next(input)
}

pub fn keyword_parser_ignore<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("ignore").parse_next(input)
}

pub fn keyword_parser_immediate<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("immediate").parse_next(input)
}

pub fn keyword_parser_in<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("in").parse_next(input)
}

pub fn keyword_parser_index<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("index").parse_next(input)
}

pub fn keyword_parser_insert<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("insert").parse_next(input)
}

pub fn keyword_parser_instead<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("instead").parse_next(input)
}

pub fn keyword_parser_into<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("into").parse_next(input)
}

pub fn keyword_parser_isnull<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("isnull").parse_next(input)
}

pub fn keyword_parser_like<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("like").parse_next(input)
}

pub fn keyword_parser_limit<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("limit").parse_next(input)
}

pub fn keyword_parser_match<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("match").parse_next(input)
}

pub fn keyword_parser_not<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("not").parse_next(input)
}

pub fn keyword_parser_notnull<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("notnull").parse_next(input)
}

pub fn keyword_parser_null<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("null").parse_next(input)
}

pub fn keyword_parser_of<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("of").parse_next(input)
}

pub fn keyword_parser_offset<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("offset").parse_next(input)
}

pub fn keyword_parser_on<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("on").parse_next(input)
}

pub fn keyword_parser_or<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("or").parse_next(input)
}

pub fn keyword_parser_order<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("order").parse_next(input)
}

pub fn keyword_parser_plan<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("plan").parse_next(input)
}

pub fn keyword_parser_pragma<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("pragma").parse_next(input)
}

pub fn keyword_parser_query<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("query").parse_next(input)
}

pub fn keyword_parser_recursive<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("recursive").parse_next(input)
}

pub fn keyword_parser_regex<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("regex").parse_next(input)
}

pub fn keyword_parser_reindex<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("reindex").parse_next(input)
}

pub fn keyword_parser_release<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("release").parse_next(input)
}

pub fn keyword_parser_rename<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("rename").parse_next(input)
}

pub fn keyword_parser_replace<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("replace").parse_next(input)
}

pub fn keyword_parser_returning<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("returning").parse_next(input)
}

pub fn keyword_parser_rollback<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("rollback").parse_next(input)
}

pub fn keyword_parser_row<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("row").parse_next(input)
}

pub fn keyword_parser_savepoint<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("savepoint").parse_next(input)
}

pub fn keyword_parser_select<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("select").parse_next(input)
}

pub fn keyword_parser_set<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("set").parse_next(input)
}

pub fn keyword_parser_table<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("table").parse_next(input)
}

pub fn keyword_parser_temp<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("temp").parse_next(input)
}

pub fn keyword_parser_temporary<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("temporary").parse_next(input)
}

pub fn keyword_parser_then<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("then").parse_next(input)
}

pub fn keyword_parser_to<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("to").parse_next(input)
}

pub fn keyword_parser_transaction<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("transaction").parse_next(input)
}

pub fn keyword_parser_trigger<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("trigger").parse_next(input)
}

pub fn keyword_parser_unique<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("unique").parse_next(input)
}

pub fn keyword_parser_update<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("update").parse_next(input)
}

pub fn keyword_parser_using<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("using").parse_next(input)
}

pub fn keyword_parser_vacuum<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("vacuum").parse_next(input)
}

pub fn keyword_parser_values<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("values").parse_next(input)
}

pub fn keyword_parser_view<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("view").parse_next(input)
}

pub fn keyword_parser_virtual<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("virtual").parse_next(input)
}

pub fn keyword_parser_when<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("when").parse_next(input)
}

pub fn keyword_parser_where<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("where").parse_next(input)
}

pub fn keyword_parser_window<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("window").parse_next(input)
}

pub fn keyword_parser_with<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("with").parse_next(input)
}
