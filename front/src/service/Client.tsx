import mock from "../mock/info-mock.json";

export default async function requestInfo(): Promise<SchemaInfo> {
  if (process.env.MOCKED == "true" || process.env.ENDPOINT_URL == undefined) {
    return mock.schema_info;
  } else {
    const data = await fetch(process.env.ENDPOINT_URL, {
      cache: "no-store",
    }).then((res) => res.json());
    const schema: SchemaInfo = data["schema_info"];
    return schema;
  }
}
