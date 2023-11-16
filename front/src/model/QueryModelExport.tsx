type QueryModelExport = {
  selectedAttribute: string;
  selectedOperator: string;
  selectedLogical: string;
  selectedValue: string | number;
  subqueries: QueryModelExport[] | undefined;
};
