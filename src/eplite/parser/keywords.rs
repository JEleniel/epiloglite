use winnow::{ascii::Caseless, ModalResult, Parser};

pub fn prs_abort<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("abort").parse_next(input)
}

pub fn prs_action<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("action").parse_next(input)
}

pub fn prs_add<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("add").parse_next(input)
}

pub fn prs_after<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("after").parse_next(input)
}

pub fn prs_all<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("all").parse_next(input)
}

pub fn prs_alter<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("alter").parse_next(input)
}

pub fn prs_always<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("always").parse_next(input)
}

pub fn prs_analyze<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("analyze").parse_next(input)
}

pub fn prs_and<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("and").parse_next(input)
}

pub fn prs_as<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("as").parse_next(input)
}

pub fn prs_asc<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("asc").parse_next(input)
}

pub fn prs_attach<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("attach").parse_next(input)
}

pub fn prs_autoincrement<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("autoincrement").parse_next(input)
}

pub fn prs_before<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("before").parse_next(input)
}

pub fn prs_begin<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("begin").parse_next(input)
}

pub fn prs_between<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("between").parse_next(input)
}

pub fn prs_by<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("by").parse_next(input)
}

pub fn prs_cascade<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("cascade").parse_next(input)
}

pub fn prs_case<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("case").parse_next(input)
}

pub fn prs_cast<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("cast").parse_next(input)
}

pub fn prs_check<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("check").parse_next(input)
}

pub fn prs_collate<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("collate").parse_next(input)
}

pub fn prs_column<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("column").parse_next(input)
}

pub fn prs_commit<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("commit").parse_next(input)
}

pub fn prs_conflict<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("conflict").parse_next(input)
}

pub fn prs_constraint<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("constraint").parse_next(input)
}

pub fn prs_create<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("create").parse_next(input)
}

pub fn prs_cross<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("cross").parse_next(input)
}

pub fn prs_current<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("current").parse_next(input)
}

pub fn prs_current_date<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("current_date").parse_next(input)
}

pub fn prs_current_time<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("current_time").parse_next(input)
}

pub fn prs_current_timestamp<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("current_timestamp").parse_next(input)
}

pub fn prs_database<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("database").parse_next(input)
}

pub fn prs_default<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("default").parse_next(input)
}

pub fn prs_deferrable<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("deferrable").parse_next(input)
}

pub fn prs_deferred<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("deferred").parse_next(input)
}

pub fn prs_delete<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("delete").parse_next(input)
}

pub fn prs_desc<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("desc").parse_next(input)
}

pub fn prs_detach<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("detach").parse_next(input)
}

pub fn prs_distinct<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("distinct").parse_next(input)
}

pub fn prs_do<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("do").parse_next(input)
}

pub fn prs_drop<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("drop").parse_next(input)
}

pub fn prs_each<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("each").parse_next(input)
}

pub fn prs_else<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("else").parse_next(input)
}

pub fn prs_end<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("end").parse_next(input)
}

pub fn prs_escape<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("escape").parse_next(input)
}

pub fn prs_except<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("except").parse_next(input)
}

pub fn prs_exclude<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("exclude").parse_next(input)
}

pub fn prs_exclusive<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("exclusive").parse_next(input)
}

pub fn prs_exists<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("exists").parse_next(input)
}

pub fn prs_explain<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("explain").parse_next(input)
}

pub fn prs_fail<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("fail").parse_next(input)
}

pub fn prs_filter<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("filter").parse_next(input)
}

pub fn prs_first<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("first").parse_next(input)
}

pub fn prs_following<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("following").parse_next(input)
}

pub fn prs_for<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("for").parse_next(input)
}

pub fn prs_foreign<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("foreign").parse_next(input)
}

pub fn prs_from<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("from").parse_next(input)
}

pub fn prs_full<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("full").parse_next(input)
}

pub fn prs_generated<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("generated").parse_next(input)
}

pub fn prs_glob<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("glob").parse_next(input)
}

pub fn prs_group<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("group").parse_next(input)
}

pub fn prs_groups<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("groups").parse_next(input)
}

pub fn prs_having<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("having").parse_next(input)
}

pub fn prs_if<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("if").parse_next(input)
}

pub fn prs_ignore<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("ignore").parse_next(input)
}

pub fn prs_immediate<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("immediate").parse_next(input)
}

pub fn prs_in<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("in").parse_next(input)
}

pub fn prs_index<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("index").parse_next(input)
}

pub fn prs_indexed<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("indexed").parse_next(input)
}

pub fn prs_initially<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("initially").parse_next(input)
}

pub fn prs_inner<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("inner").parse_next(input)
}

pub fn prs_insert<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("insert").parse_next(input)
}

pub fn prs_instead<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("instead").parse_next(input)
}

pub fn prs_intersect<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("intersect").parse_next(input)
}

pub fn prs_into<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("into").parse_next(input)
}

pub fn prs_is<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("is").parse_next(input)
}

pub fn prs_isnull<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("isnull").parse_next(input)
}

pub fn prs_join<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("join").parse_next(input)
}

pub fn prs_key<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("key").parse_next(input)
}

pub fn prs_last<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("last").parse_next(input)
}

pub fn prs_left<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("left").parse_next(input)
}

pub fn prs_like<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("like").parse_next(input)
}

pub fn prs_limit<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("limit").parse_next(input)
}

pub fn prs_match<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("match").parse_next(input)
}

pub fn prs_materialized<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("materialized").parse_next(input)
}

pub fn prs_natural<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("natural").parse_next(input)
}

pub fn prs_no<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("no").parse_next(input)
}

pub fn prs_not<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("not").parse_next(input)
}

pub fn prs_nothing<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("nothing").parse_next(input)
}

pub fn prs_notnull<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("notnull").parse_next(input)
}

pub fn prs_null<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("null").parse_next(input)
}

pub fn prs_nulls<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("nulls").parse_next(input)
}

pub fn prs_of<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("of").parse_next(input)
}

pub fn prs_offset<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("offset").parse_next(input)
}

pub fn prs_on<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("on").parse_next(input)
}

pub fn prs_or<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("or").parse_next(input)
}

pub fn prs_order<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("order").parse_next(input)
}

pub fn prs_others<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("others").parse_next(input)
}

pub fn prs_outer<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("outer").parse_next(input)
}

pub fn prs_over<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("over").parse_next(input)
}

pub fn prs_partition<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("partition").parse_next(input)
}

pub fn prs_plan<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("plan").parse_next(input)
}

pub fn prs_pragma<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("pragma").parse_next(input)
}

pub fn prs_preceding<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("preceding").parse_next(input)
}

pub fn prs_primary<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("primary").parse_next(input)
}

pub fn prs_query<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("query").parse_next(input)
}

pub fn prs_raise<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("raise").parse_next(input)
}

pub fn prs_range<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("range").parse_next(input)
}

pub fn prs_recursive<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("recursive").parse_next(input)
}

pub fn prs_references<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("references").parse_next(input)
}

pub fn prs_regexp<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("regexp").parse_next(input)
}

pub fn prs_reindex<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("reindex").parse_next(input)
}

pub fn prs_release<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("release").parse_next(input)
}

pub fn prs_rename<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("rename").parse_next(input)
}

pub fn prs_replace<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("replace").parse_next(input)
}

pub fn prs_restrict<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("restrict").parse_next(input)
}

pub fn prs_returning<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("returning").parse_next(input)
}

pub fn prs_right<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("right").parse_next(input)
}

pub fn prs_rollback<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("rollback").parse_next(input)
}

pub fn prs_row<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("row").parse_next(input)
}

pub fn prs_rows<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("rows").parse_next(input)
}

pub fn prs_savepoint<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("savepoint").parse_next(input)
}

pub fn prs_select<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("select").parse_next(input)
}

pub fn prs_set<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("set").parse_next(input)
}

pub fn prs_table<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("table").parse_next(input)
}

pub fn prs_temp<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("temp").parse_next(input)
}

pub fn prs_temporary<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("temporary").parse_next(input)
}

pub fn prs_then<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("then").parse_next(input)
}

pub fn prs_ties<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("ties").parse_next(input)
}

pub fn prs_to<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("to").parse_next(input)
}

pub fn prs_transaction<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("transaction").parse_next(input)
}

pub fn prs_trigger<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("trigger").parse_next(input)
}

pub fn prs_unbounded<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("unbounded").parse_next(input)
}

pub fn prs_union<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("union").parse_next(input)
}

pub fn prs_unique<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("unique").parse_next(input)
}

pub fn prs_update<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("update").parse_next(input)
}

pub fn prs_using<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("using").parse_next(input)
}

pub fn prs_vacuum<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("vacuum").parse_next(input)
}

pub fn prs_values<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("values").parse_next(input)
}

pub fn prs_view<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("view").parse_next(input)
}

pub fn prs_virtual<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("virtual").parse_next(input)
}

pub fn prs_when<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("when").parse_next(input)
}

pub fn prs_where<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("where").parse_next(input)
}

pub fn prs_window<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("window").parse_next(input)
}

pub fn prs_with<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("with").parse_next(input)
}

pub fn prs_without<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    Caseless("without").parse_next(input)
}
