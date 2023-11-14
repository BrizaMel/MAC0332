import { v4 } from "uuid";

export default function updateUUID(query: QueryModel) {
  return { ...query, id: v4() };
}
