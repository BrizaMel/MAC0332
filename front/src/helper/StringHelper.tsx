export function convertReadableStringToPath(str: string): string {
  return str
    .replaceAll(" em ", ".")
    .replaceAll(" ", "")
    .split(".")
    .reverse()
    .join(".");
}

export function convertPathToReadableString(path: string): string {
  return path.split(".").reverse().join(" em ");
}

export function generateValidProjection(fields: string[]): string[] {
  return fields.map((field) => convertReadableStringToPath(field));
}

export function generateStringFromQueryArray(
  query: QueryModelExport[]
): string {
  var base = "(";
  query.forEach((q) => {
    base += `${convertReadableStringToPath(q.selectedAttribute)} ${
      q.selectedOperator
    } ${q.selectedValue} ${
      q.subqueries != undefined && q.subqueries.length > 0
        ? q.selectedLogicalSubquerie
        : ""
    } ${q.subqueries ? generateStringFromQueryArray(q.subqueries) : ""}  ${
      q.selectedLogical ?? ""
    } `;
  });
  base += " )";
  return base.replace(/  +/g, " ").trim();
}
