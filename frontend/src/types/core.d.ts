declare const __brand: unique symbol;

declare interface Brand<B> {
  [__brand]: B;
}

declare type Branded<T, B> = T & Brand<B>;
