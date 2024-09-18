export const Rule = {
  REQUIRED: (value: unknown) => (value !== null && value !== undefined) || 'Field is required',
  NON_EMPTY: (value: unknown[]) => value.length > 0 || 'Field should not be empty',
};

export class DialogUtils<T> {
  private resolver!: (value: T) => void;

  /**
   * Get the response via a Promise. This allows you to do an action when the response is made.
   * Must call getDialogResponse() BEFORE the user has an opportunity to call setDialogResponse() otherwise it will fail.
   * @returns
   */
  public getDialogResponse(): Promise<T> {
    return new Promise((resolve) => {
      this.resolver = resolve;
    });
  }

  /**
   * The dialog calls this function to set the response selected by the user.
   * @param response
   */
  public setDialogResponse(response: T): void {
    this.resolver(response);
  }
}
