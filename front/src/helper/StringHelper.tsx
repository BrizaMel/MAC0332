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
  var base = "";
  query.forEach((q) => {
    base += `${convertReadableStringToPath(q.selectedAttribute)} ${
      translateOperator(q.selectedOperator)
    } ${q.selectedValue} ${
      q.subqueries != undefined && q.subqueries.length > 0
        ? q.selectedLogicalSubquerie
        : ""
    } ${q.subqueries ? `(${generateStringFromQueryArray(q.subqueries)})` : ""}  ${
      q.selectedLogical ?? ""
    } `;
  });
  return base.replace(/  +/g, " ").trim();
}

function translateOperator(operator: string): string{
  let back_operator;

  switch(operator) {
  case "EqualTo":
    back_operator = "eq";
    break;
  case "GreaterThan":
    back_operator = "gt";
    break;
  case "LessThan":
    back_operator = "lt";
    break;
  case "GreaterThanOrEqualTo":
    back_operator = "ge";
    break;
  case "LessThanOrEqualTo":
    back_operator = "le";
    break;
  case "NotEqualTo":
    back_operator = "ne";
    break;
  default:
    back_operator = "UNKOWN";
  }

  return back_operator;

}