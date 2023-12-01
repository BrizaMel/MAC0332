import updateUUID from "@/helper/UUIDHandler";
import { useEffect, useState } from "react";
import Autocomplete from "@mui/material/Autocomplete";
import { TextField } from "@mui/material";

export default function SelectComponent({
  queryParam,
  isLast,
  handleDelete,
  schemaInfoParam,
}: any) {
  const query: QueryModel = queryParam;
  const schemaInfo: SchemaInfo = schemaInfoParam;
  const [subqueries, setSubqueries] = useState<QueryModel[] | undefined>();

  useEffect(() => {
    query.subQueries = subqueries;
  }, [query, subqueries]);

  function handleSelectedAttribute(value: string) {
    query.selectedAttribute = value;
  }

  function handleSelectedInput(inputText: string) {
    query.selectedInput = inputText;
  }

  function handleSelectedOperator(value: string) {
    query.selectedOperator = value;
  }

  function addSubqueries() {
    subqueries == undefined
      ? setSubqueries([updateUUID(query)])
      : setSubqueries([...subqueries, updateUUID(query)]);
  }

  function handleDeleteFromChild(query: QueryModel) {
    setSubqueries(
      subqueries!.filter((q) => {
        return q.id != query.id;
      })
    );
  }

  function handleSelectedLogical(value: string) {
    query.selectedLogical = value;
  }

  function handleSelectedLogicalSubquerie(value: string) {
    query.selectedLogicalSubquerie = value;
  }

  return (
    <div className="select-component">
      <h3>Query</h3> <br />
      <div style={{ display: "flex", alignItems: "center" }}>
        <Autocomplete
          className="select-attr"
          options={schemaInfo.attributes.map((attr) => attr.name)}
          onChange={(event: any, newValue: string | null) => {
            handleSelectedAttribute(newValue ?? "");
          }}
          popupIcon={""}
          clearIcon={""}
          renderInput={(params) => (
            <TextField {...params} label={"Procure um atributo"} />
          )}
          renderOption={(props, option) => {
            return (
              <li {...props} key={option}>
                {option}
              </li>
            );
          }}
        />
        <select
          className="logical-select"
          onChange={(e) => handleSelectedOperator(e.target.value)}
        >
          <option></option>
          {schemaInfo.operators.map((op) => (
            <option key={op} value={op}>
              {op}
            </option>
          ))}
        </select>
        <Autocomplete
          freeSolo
          className="select-attr"
          popupIcon={""}
          clearIcon={""}
          options={schemaInfo.attributes.map((attr) => attr.name)}
          onInputChange={(event: any, newValue: string | null) => {
            handleSelectedInput(newValue ?? "");
          }}
          renderInput={(params) => (
            <TextField {...params} label="Digite um valor ou campo" />
          )}
          renderOption={(props, option) => {
            return (
              <li {...props} key={option}>
                {option}
              </li>
            );
          }}
        />
        <button onClick={(e) => handleDelete(query)}>delete</button>
      </div>
      {subqueries && subqueries.length > 0 && (
        <select
          onChange={(e) => handleSelectedLogicalSubquerie(e.target.value)}
          className="logical-select"
          style={{ marginLeft: 30, marginTop: 40 }}
        >
          <option value="empty"></option>
          {schemaInfo.logical_operators.map((op) => (
            <option key={op} value={op}>
              {op}
            </option>
          ))}
        </select>
      )}
      {subqueries?.map((subquery, index) => (
        <div style={{ marginLeft: 20 }} key={subquery.id}>
          <SelectComponent
            key={subquery.id}
            handleDelete={handleDeleteFromChild}
            queryParam={subquery}
            schemaInfoParam={schemaInfo}
            isLast={index == subqueries.length - 1}
          />
        </div>
      ))}
      <button onClick={addSubqueries}>
        add {subqueries && subqueries.length > 0 ? "to" : ""} group
      </button>
      <br />
      {!isLast && (
        <select
          onChange={(e) => handleSelectedLogical(e.target.value)}
          className="logical-select"
          style={{ marginTop: 40 }}
        >
          <option value="empty"></option>
          {schemaInfo.logical_operators.map((op) => (
            <option key={op} value={op}>
              {op}
            </option>
          ))}
        </select>
      )}
    </div>
  );
}
