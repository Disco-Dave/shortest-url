type Props = {
  url: string;
  urlError?: string;
  onUrlChange: (newUrl: string) => void;
  onUrlBlur: () => void;
  onSubmit: () => void;
  onReset: () => void;
  isLoading?: boolean;
};

export default function UrlForm(props: Props) {
  function handleSubmit(event: { preventDefault: () => void }) {
    event.preventDefault();
    props.onSubmit();
  }

  return (
    <form onSubmit={handleSubmit}>
      <div className={"field " + (props.urlError ? "field--invalid" : "")}>
        <label htmlFor="url" className="field__label">URL</label>
        <input
          className="field__input"
          id="url"
          name="url"
          type="text"
          value={props.url}
          onChange={(e) => props.onUrlChange(e.currentTarget.value)}
          onBlur={props.onUrlBlur}
        />
        {props.urlError && <p className="field__feedback">{props.urlError}</p>}
      </div>
      <div>
        <button id="shorten-url" type="submit" className="button">
          {props.isLoading ? <div className="lds-dual-ring"></div> : "Shorten"}

        </button>
        <button id="reset" type="reset" onClick={props.onReset} className="button button--danger">
          Reset
        </button>
      </div>
    </form>
  );
}
