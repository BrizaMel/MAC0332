import infoMock from "../mock/info-mock.json";
import postMock from "../mock/request-post-mock.json";

export async function requestInfo(): Promise<SchemaInfo> {
  if (process.env.MOCKED == "true" || process.env.ENDPOINT_URL == undefined) {
    return infoMock.schema_info;
  } else {
    const data = await fetch(`${process.env.ENDPOINT_URL}/properties`, {
      cache: "no-store",
    }).then((res) => res.json());
    const schema: SchemaInfo = data["schema_info"];
    return schema;
  }
}

export async function sendQueryRequest(
  body: RequestModel
): Promise<Array<any>> {
  if (process.env.MOCKED == "true" || process.env.ENDPOINT_URL == undefined) {
    return postMock;
  } else {
    console.log(body);
    const data = await fetch(`${process.env.ENDPOINT_URL}/search`, {
      cache: "no-store",
      method: "POST",
      body: JSON.stringify(body),
    }).then((res) => res.json());
    return data;
  }
}
