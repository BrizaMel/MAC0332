type SchemaInfo = {
  attributes: Attribute[];
  subsets: number[][];
  operators: string[];
  logical_operators: string[];
};

type Attribute = {
  name: string;
  type: string;
  subset: number;
};
