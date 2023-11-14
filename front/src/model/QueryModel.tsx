type QueryModel = {
  id: string;
  selectedAttribute?: string;
  selectedOperator?: string;
  selectedInput?: string;
  selectedLogical?: string;
  subQueries?: QueryModel[];
};
