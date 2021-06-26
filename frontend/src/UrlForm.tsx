type Props = {
  url: string;
  urlError?: string;
  onUrlChange: (newUrl: string) => void;
  onSubmit: () => void;
  onReset: () => void;
};

export default function UrlForm(props: Props) {
  function handleSubmit(event: { preventDefault: () => void }) {
    event.preventDefault();
    props.onSubmit();
  }

  return (
    <form onSubmit={handleSubmit}>
      <div>
        <label htmlFor="url">URL</label>
        <input
          id="url"
          name="url"
          type="text"
          value={props.url}
          onChange={(e) => props.onUrlChange(e.currentTarget.value)}
        />
        {props.urlError && <p>{props.urlError}</p>}
      </div>
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
