import * as React from "react";
import * as api from "./api";
import ShortUrl from "./ShortUrl";
import UrlForm from "./UrlForm";

export default function App() {
  const [isLoading, setIsLoading] = React.useState(false);
  const [url, setUrl] = React.useState("");
  const [shortUrl, setShortUrl] = React.useState("");
  const [urlError, setUrlError] = React.useState("");

  async function handleSubmit() {
    handleUrlBlur();

    const trimmedUrl = url.trim();

    if (trimmedUrl && !urlError && !isLoading) {
      try {
        setUrlError("");
        setIsLoading(true);

        const result = await api.postUrl(trimmedUrl);
        if (result.kind === "right") {
          setShortUrl(api.getUrl(result.value));
        } else {
          setUrlError(result.value);
        }
      } finally {
        setIsLoading(false);
      }
    }
  }

  function handleUrlBlur() {
    if (url.trim() === "")  {
      setUrlError("Required");
    }
    else {
      setUrlError("");

      try {
        new URL(url);
      }
      catch {
        setUrlError("URL is invalid")
      }
    }
  }

  function handleReset() {
    setUrl("");
    setShortUrl("");
    setUrlError("");
  }

  return (
    <main className="main">
      <h1 className="title">URL Shortener</h1>
      <UrlForm
        url={url}
        urlError={urlError}
        onUrlChange={setUrl}
        onUrlBlur={handleUrlBlur}
        onReset={handleReset}
        onSubmit={handleSubmit}
        isLoading={isLoading}
      />
      <ShortUrl shortUrl={shortUrl} />
    </main>
  );
}
