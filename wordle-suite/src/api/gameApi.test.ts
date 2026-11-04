import { describe, expect, it } from "vitest";
import { handleApiError } from "./gameApi";

describe("handleApiError", () => {
  it("returns a safe message for Error instances", () => {
    expect(handleApiError(new Error("request failed"))).toBe("request failed");
  });

  it("does not expose unknown error values", () => {
    expect(handleApiError({ secret: "redacted" })).toBe("An unknown error occurred");
  });
});
