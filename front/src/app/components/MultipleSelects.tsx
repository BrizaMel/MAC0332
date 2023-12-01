import { TextField } from "@mui/material";
import Autocomplete from "@mui/material/Autocomplete";

export default function MultipleSelect({ values, handleProjection }: any) {
  return (
    <div>
      {values != undefined && (
        <Autocomplete
          multiple
          className="select-attr multiple-select"
          options={values.map((attr: Attribute) => attr.name)}
          filterSelectedOptions
          renderInput={(params) => (
            <TextField {...params} label="Selecione os campos" />
          )}
          onChange={(event, newValue) => {
            const ns = newValue as string[];
            handleProjection(ns);
          }}
          popupIcon={""}
          clearIcon={""}
        />
      )}
    </div>
  );
}
