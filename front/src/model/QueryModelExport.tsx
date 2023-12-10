type QueryModelExport = {
  selectedAttribute: string;
  selectedOperator: string;
  selectedLogical: string;
  selectedLogicalSubquerie: string;
  selectedValue: string | number;
  subqueries: QueryModelExport[] | undefined;
};
