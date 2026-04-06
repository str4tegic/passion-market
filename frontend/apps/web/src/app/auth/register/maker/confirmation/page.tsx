import Link from 'next/link'

export default function RegisterMakerConfirmationPage() {
  return (
    <main>
      <h1>Compte créé — en attente de validation</h1>
      <p>Votre compte Maker a été créé avec succès. Notre équipe va valider votre inscription.</p>
      <Link href="/">Retour à l&apos;accueil</Link>
    </main>
  )
}
