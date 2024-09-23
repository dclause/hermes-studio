declare const __brand: unique symbol;

declare interface Brand<B> {
  [__brand]: B;
}

export declare type Branded<T, B> = T & Brand<B>;

export declare type Range<T> = [T, T];

export declare interface Entity<Id> {
  id: Id;
  name: string;
  loading?: bool;
}
