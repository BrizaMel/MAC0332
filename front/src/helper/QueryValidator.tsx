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

export function validateQueries(queries: QueryModel[]): boolean {
  if (queries.length <= 0) return false;
  let status = true;
  queries.forEach((q) => {
    if (!validateQuery(q)) status = false;
  });
  return status;
}

function validateQuery(query: QueryModel): boolean {
  if (query.selectedAttribute == undefined || query.selectedAttribute == "")
    return false;

  if (query.selectedOperator == undefined || query.selectedOperator == "")
    return false;

  if (
    query.subQueries != undefined &&
    query.subQueries.length > 0 &&
    query.selectedLogicalSubquerie == undefined
  )
    return false;

  if (
    query.subQueries != undefined &&
    query.subQueries.length > 0 &&
    !validateQueries(query.subQueries)
  ) {
    return false;
  }
  return true;
}
