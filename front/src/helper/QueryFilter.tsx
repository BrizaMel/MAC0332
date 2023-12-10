import { convertReadableStringToPath } from "./StringHelper";

export default function getSelectedAttributesFromQueries(
  queries: QueryModel[],
  schema: SchemaInfo
): QueryModelExport[] {
  return queries.map((q, index) =>
    getSelectedAttributesFromQuery(q, schema, index == queries.length - 1)
  );
}

function getSelectedAttributesFromQuery(
  query: QueryModel,
  schema: SchemaInfo,
  isLast: boolean
): QueryModelExport {
  return {
    selectedAttribute: query.selectedAttribute!,
    selectedOperator: query.selectedOperator!,
    selectedValue: isAttribute(query.selectedInput!, schema)
      ? convertReadableStringToPath(query.selectedInput!)
      : query.selectedInput!,
    selectedLogical: isLast ? "" : query.selectedLogical!,
    selectedLogicalSubquerie: query.selectedLogicalSubquerie ?? "",
    subqueries:
      query.subQueries && query.subQueries.length > 0
        ? getSelectedAttributesFromQueries(query.subQueries, schema)
        : undefined,
  };
}

function isAttribute(text: string, schema: SchemaInfo) {
  let res = false;
  for (let attr of schema.attributes) {
    if (attr.name == text) {
      res = true;
      break;
    }
  }
  return res;
}
