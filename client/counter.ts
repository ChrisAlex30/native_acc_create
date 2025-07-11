export class CounterState {
  count: number;

  constructor(props: { count: number }) {
    this.count = props.count;
  }
}

export const Schema = new Map([
  [CounterState, { kind: 'struct', fields: [['count', 'u32']] }]
]);
