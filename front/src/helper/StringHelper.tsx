function convertReadableStringToPath(str: string): string {
  return str
    .replaceAll(" em ", ".")
    .replaceAll(" ", "")
    .split(".")
    .reverse()
    .join(".");
}

export function generateStringFromQueryArray(
  query: QueryModelExport[]
): string {
  // TO-DO
  // const isNumber = query[0].selectedValue.toString().match("")

  var base = "(";
  query.forEach((q) => {
    base += `${convertReadableStringToPath(q.selectedAttribute)} ${
      q.selectedOperator
    } "${q.selectedValue}" ${
      q.subqueries != undefined && q.subqueries.length > 0
        ? q.selectedLogicalSubquerie
        : ""
    } ${q.subqueries ? generateStringFromQueryArray(q.subqueries) : ""}  ${
      q.selectedLogical ?? ""
    } `;
  });
  base += " )";
  return base.replace(/  +/g, " ");
}
