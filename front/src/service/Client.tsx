import infoMock from "../mock/info-mock.json";
import postMock from "../mock/request-post-mock.json";

export async function requestInfo(): Promise<SchemaInfo> {
  if (process.env.MOCKED == "true" || process.env.ENDPOINT_URL == undefined) {
    return infoMock.schema_info;
  } else {
    const data = await fetch(`${process.env.ENDPOINT_URL}/properties`, {
      cache: "no-store",
    }).then((res) => res.json());
    const schema: SchemaInfo = data["properties"];
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
      headers : {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': 'http://localhost:3001',
        'Access-Control-Allow-Headers': '*',
      },
      body: JSON.stringify(body),
    })  .then((res) => res.json());
    let search_result = JSON.parse(data)["search_result"];
    return search_result;
  }
}
