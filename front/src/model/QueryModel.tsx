type QueryModel = {
  id: string;
  selectedAttribute?: string;
  selectedOperator?: string;
  selectedInput?: string | number;
  selectedLogical?: string;
  selectedLogicalSubquerie?: string;
  subQueries?: QueryModel[];
};
