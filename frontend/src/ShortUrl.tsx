import * as React from "react";

type Props = { shortUrl: string };

export default function ShortUrl(props: Props) {
  const [clipboardMessage, setClipboardMessage] = React.useState<string>("");

  React.useEffect(() => {
    setClipboardMessage("");
  }, [props.shortUrl]);

  async function handleCopy() {
    if (props.shortUrl) {
      await navigator.clipboard.writeText(props.shortUrl);
      setClipboardMessage("URL copied to clipboard");
      setTimeout(() => {
        setClipboardMessage("");
      }, 5000);
    }
  }

  return (
    <div>
      <label htmlFor="shortened-url">Short URL</label>
      <input
        id="short-url"
        name="short-url"
        type="text"
        readOnly
        value={props.shortUrl}
        onClick={handleCopy}
      />
      {clipboardMessage && <span>{clipboardMessage}</span>}
    </div>
  );
}
