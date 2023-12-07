"use client";

import { useEffect, useState } from "react";
import SelectComponent from "./components/SelectComponent";
import updateUUID from "@/helper/UUIDHandler";
import { validateProjection, validateQueries } from "@/helper/QueryValidator";
import getSelectedAttributesFromQueries from "@/helper/QueryFilter";
import { requestInfo, sendQueryRequest } from "@/service/Client";
import MultipleSelect from "./components/MultipleSelects";
import { generateStringFromQueryArray } from "@/helper/StringHelper";
import { QueryComponentColor } from "@/model/QueryComponentColor";

export default function Home() {
  const [schemaInfo, setSchemaInfo] = useState<SchemaInfo>();
  const [colorHandler, _updateColorHandler] = useState<QueryComponentColor>(
    QueryComponentColor.createBaseColor()
  );
  const [response, setResponse] = useState<Array<Object>>([]);

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
    if (!validateQueries(queries) || !validateProjection(projection)) {
      alert("Preencha os campos corretamente!");
      return;
    }
    const queriesToSave = getSelectedAttributesFromQueries(queries);
    const querieString = generateStringFromQueryArray(queriesToSave);
    
    const toSave = {
      projection: projection,
      filters: querieString,
    } as RequestModel;

    const reqResponse = sendQueryRequest(toSave);
    Promise.resolve(reqResponse).then((value) => setResponse(value));
  }

  return (
    <main>
      <h1 id="header-name">Serviço de Busca Complexa em Banco de Dados </h1>
      <div className="main-container">
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
            componentColor={colorHandler.createChildColor()}
          />
        ))}
        <button onClick={addQueries}>ADD</button>
        <button onClick={save}>SAVE</button>

        {response.length > 0 && (
          <table id="results-table">
            <tr id="first-row">
              {Object.keys(response[0]).map((key) => (
                <td key={key}>{key}</td>
              ))}
            </tr>
            {response.map((obj, index) => {
              return (
                <tr key={index} className="row-container">
                  {Object.keys(obj).map((key) => {
                    type ObjectKey = keyof typeof obj;
                    const k = key as ObjectKey;
                    return (
                      <td key={key}>
                        {JSON.stringify(obj[k]).replaceAll('"', "")}
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </table>
        )}
      </div>
    </main>
  );
}
