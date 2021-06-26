export type Either<TLeft, TRight> =
  | { kind: "left"; value: TLeft }
  | { kind: "right"; value: TRight };

export async function postUrl(url: string): Promise<Either<string, string>> {
  const response = await fetch("/api/", {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(url),
  });

  if (response.ok) {
    const slug = await response.json().then((data) => data as string);
    return { kind: "right", value: slug };
  } else {
    const error = await response.json().then((data) => data as string);
    return { kind: "left", value: error };
  }
}
