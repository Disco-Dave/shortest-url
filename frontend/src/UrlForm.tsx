import * as React from "react";

type Props = {
  url: string;
  onUrlChange: (newUrl: string) => void;
  onSubmit: () => void;
  onReset: () => void;
};

export default function UrlForm(props: Props) {
  const handleSubmit: React.FormEventHandler<HTMLFormElement> = (event) => {
    event.preventDefault();
    props.onSubmit();
  };

  return (
    <form onSubmit={handleSubmit}>
      <label htmlFor="url">URL</label>
      <input
        id="url"
        name="url"
        type="text"
        value={props.url}
        onChange={(e) => props.onUrlChange(e.currentTarget.value)}
      />
      <div>
        <button id="shorten-url" type="submit">
          Shorten
        </button>
        <button id="reset" type="reset" onClick={props.onReset}>
          Reset
        </button>
      </div>
    </form>
  );
}
