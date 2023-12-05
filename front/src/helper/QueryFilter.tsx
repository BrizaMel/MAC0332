export default function getSelectedAttributesFromQueries(
  queries: QueryModel[]
): QueryModelExport[] {
  return queries.map((q, index) =>
    getSelectedAttributesFromQuery(q, index == queries.length - 1)
  );
}

function getSelectedAttributesFromQuery(
  query: QueryModel,
  isLast: boolean
): QueryModelExport {
  return {
    selectedAttribute: query.selectedAttribute!,
    selectedOperator: query.selectedOperator!,
    selectedValue: query.selectedInput!,
    selectedLogical: isLast ? "" : query.selectedLogical!,
    selectedLogicalSubquerie: query.selectedLogicalSubquerie ?? "",
    subqueries:
      query.subQueries && query.subQueries.length > 0
        ? getSelectedAttributesFromQueries(query.subQueries)
        : undefined,
  };
}
