"use client";

import { useState } from "react";
import SelectComponent from "./components/SelectComponent";
import updateUUID from "@/helper/UUIDHandler";
import { validateQueries } from "@/helper/QuerySaver";
import getSelectedAttributesFromQueries from "@/helper/QuerySaver";
import { download } from "@/helper/Downloader";
import mock from "../mock/info-mock.json";

function getFile(): SchemaInfo {
  return mock.schema_info as SchemaInfo;
}

export default function Home() {
  const [schemaInfo, setSchemaInfo] = useState<SchemaInfo>(getFile());

  const [queries, setQueries] = useState<QueryModel[]>([]);

  function addQueries() {
    setQueries([...queries, updateUUID(queries[queries.length - 1])]);
  }

  function handleDeleteFromChild(query: QueryModel) {
    setQueries(
      queries.filter((q) => {
        return q.id != query.id;
      })
    );
  }

  function save() {
    if (!validateQueries(queries)) {
      console.log("INVALID");
      return;
    }
    const toSave = getSelectedAttributesFromQueries(queries);

    const dict = JSON.stringify(toSave);
    download(dict, "query.json");
  }

  return (
    <main>
      {queries.map((query, index) => (
        <SelectComponent
          key={query.id}
          handleDelete={handleDeleteFromChild}
          queryParam={query}
          schemaInfoParam={schemaInfo}
          isLast={index == queries.length - 1}
        />
      ))}
      <button onClick={addQueries}>ADD</button>
      <button onClick={save}>SAVE</button>
    </main>
  );
}
