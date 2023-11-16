type QueryModel = {
  id: string;
  selectedAttribute?: string;
  selectedOperator?: string;
  selectedInput?: string | number;
  selectedLogical?: string;
  subQueries?: QueryModel[];
};
