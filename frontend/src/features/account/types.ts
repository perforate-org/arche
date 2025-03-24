import { validate, HexaUrlError } from "hexaurl-validate";

export function validateId(id: string): string | undefined {
  try {
    validate(id);
  } catch (err) {
    if (err instanceof HexaUrlError) {
      return err.message;
    } else {
      throw err;
    }
  }
  return undefined;
}

export function validateName(name: string): string | undefined {
  if (name.length > 50) {
    return "Name must not exceed 50 characters";
  }
  return undefined;
}
