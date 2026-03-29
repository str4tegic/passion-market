/**
 * VO d'affichage pour les montants monétaires.
 *
 * Règle : les montants viennent du backend en centimes (entiers).
 * Ce VO ne contient QUE de la logique d'affichage — jamais de règle métier.
 * Les règles métier (commission, réduction) restent dans Rust order-domain.
 */
export class MoneyDisplay {
  constructor(
    private readonly cents: number,
    private readonly currency: string = 'EUR',
  ) {
    if (!Number.isFinite(cents) || cents < 0 || !Number.isInteger(cents)) {
      throw new RangeError(`MoneyDisplay: cents doit être un entier positif ou nul, reçu: ${cents}`)
    }
  }

  /** "29,90 €" */
  format(): string {
    return new Intl.NumberFormat('fr-FR', {
      style: 'currency',
      currency: this.currency,
    }).format(this.cents / 100)
  }

  /** "29.90" — pour les inputs */
  toDecimalString(): string {
    return (this.cents / 100).toFixed(2)
  }

  isZero(): boolean {
    return this.cents === 0
  }

  static fromDto(cents: number, currency = 'EUR'): MoneyDisplay {
    return new MoneyDisplay(cents, currency)
  }
}
