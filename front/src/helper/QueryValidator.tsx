export function validateProjection(projection: string[]): boolean {
  return projection.length > 0;
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

  if (query.selectedInput == undefined || query.selectedAttribute == "")
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
