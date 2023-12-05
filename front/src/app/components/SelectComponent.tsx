import updateUUID from "@/helper/UUIDHandler";
import { useEffect, useState } from "react";
import Autocomplete from "@mui/material/Autocomplete";
import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  TextField,
} from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";

import IconButton from "@mui/material/IconButton";

import { QueryComponentColor } from "@/model/QueryComponentColor";

export default function SelectComponent({
  queryParam,
  isLast,
  handleDelete,
  schemaInfoParam,
  componentColor,
}: any) {
  const query: QueryModel = queryParam;
  const schemaInfo: SchemaInfo = schemaInfoParam;
  const [colorHandler, _updateColorHandler] =
    useState<QueryComponentColor>(componentColor);
  const [subqueries, setSubqueries] = useState<QueryModel[] | undefined>();

  const thisTextAccentColor = colorHandler.getTextColor();
  const thisAccentColor = colorHandler.getAccentColor();
  const thisBackgroundColor = colorHandler.getBackgroundColor();

  useEffect(() => {
    query.subQueries = subqueries;
  }, [query, subqueries]);

  function handleSelectedAttribute(value: string) {
    query.selectedAttribute = value;
  }

  function handleSelectedInput(inputText: string) {
    query.selectedInput = inputText;
  }

  function handleSelectedOperator(event: SelectChangeEvent) {
    query.selectedOperator = event.target.value;
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

  function handleSelectedLogicalSubquerie(event: SelectChangeEvent) {
    query.selectedLogicalSubquerie = event.target.value;
  }

  function handleSelectedLogical(event: SelectChangeEvent) {
    query.selectedLogical = event.target.value;
  }

  return (
    <div>
      <div
        className="select-component"
        style={{
          backgroundColor: thisBackgroundColor,
          borderColor: thisAccentColor,
        }}
      >
        <h3 style={{ color: thisTextAccentColor }}>Query</h3> <br />
        <div
          style={{ display: "flex", alignItems: "top", marginBottom: "2vh" }}
        >
          <Autocomplete
            className="select-attr"
            options={schemaInfo.attributes.map((attr) => attr.name)}
            onChange={(event: any, newValue: string | null) => {
              handleSelectedAttribute(newValue ?? "");
            }}
            popupIcon={""}
            clearIcon={""}
            forcePopupIcon={false}
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
          <FormControl>
            <InputLabel id="label-operador">Operador</InputLabel>
            <Select
              onChange={handleSelectedOperator}
              className="mui-select"
              labelId="label-operador"
            >
              {schemaInfo.operators.map((op) => (
                <MenuItem key={op} value={op}>
                  {op}
                </MenuItem>
              ))}
            </Select>
          </FormControl>

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

          <IconButton
            aria-label="delete"
            style={{ width: 40, height: 55 }}
            size="large"
            onClick={(e) => handleDelete(query)}
          >
            <DeleteIcon sx={{ color: "grey" }} fontSize="inherit" />
          </IconButton>
        </div>
        {subqueries && subqueries.length > 0 && (
          <FormControl
            style={{ marginLeft: 40, marginTop: 30, marginBottom: 20 }}
          >
            <InputLabel id="label-operador">Operador</InputLabel>
            <Select
              onChange={handleSelectedLogicalSubquerie}
              className="mui-select"
              labelId="label-operador"
            >
              {schemaInfo.logical_operators.map((op) => (
                <MenuItem key={op} value={op}>
                  {op}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        )}
        {subqueries?.map((subquery, index) => (
          <div style={{ marginLeft: 20 }} key={subquery.id}>
            <SelectComponent
              key={subquery.id}
              handleDelete={handleDeleteFromChild}
              queryParam={subquery}
              schemaInfoParam={schemaInfo}
              isLast={index == subqueries.length - 1}
              componentColor={colorHandler.createChildColor()}
            />
          </div>
        ))}
        <button onClick={addSubqueries}>
          add {subqueries && subqueries.length > 0 ? "to" : ""} group
        </button>
        <br />
      </div>

      {!isLast && (
        <FormControl
          style={{ marginLeft: 40, marginTop: 30, marginBottom: 20 }}
        >
          <InputLabel id="label-operador">Operador</InputLabel>
          <Select
            onChange={handleSelectedLogical}
            className="mui-select"
            labelId="label-operador"
          >
            {schemaInfo.logical_operators.map((op) => (
              <MenuItem key={op} value={op}>
                {op}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
      )}
    </div>
  );
}
