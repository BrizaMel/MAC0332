"use client";

import { useEffect, useState } from "react";
import SelectComponent from "./components/SelectComponent";
import updateUUID from "@/helper/UUIDHandler";
import { validateQueries } from "@/helper/QueryValidator";
import getSelectedAttributesFromQueries from "@/helper/QueryValidator";
import { download } from "@/helper/Downloader";
import requestInfo from "@/service/Client";
import MultipleSelect from "./components/MultipleSelects";
import { generateStringFromQueryArray } from "@/helper/StringHelper";

export default function Home() {
  const [schemaInfo, setSchemaInfo] = useState<SchemaInfo>();

  useEffect(() => {
    const data = requestInfo();
    Promise.resolve(data).then((value) => setSchemaInfo(value));
  }, []);

  const [queries, setQueries] = useState<QueryModel[]>([]);
  const [projection, setProjection] = useState<string[]>([]);

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
    const queriesToSave = getSelectedAttributesFromQueries(queries);
    const querieString = generateStringFromQueryArray(queriesToSave);
    const toSave = {
      projection: projection,
      filters: querieString,
    } as RequestModel;

    const dict = JSON.stringify(toSave);
    // download(dict, "query.json");
    console.log(toSave);
    console.log(queriesToSave);
  }

  return (
    <main>
      <h1>SBCBD --------</h1>
      <h1>Campos a serem visualizados</h1>
      <MultipleSelect
        values={schemaInfo?.attributes}
        handleProjection={setProjection}
      />

      <h1>Filtros</h1>
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
