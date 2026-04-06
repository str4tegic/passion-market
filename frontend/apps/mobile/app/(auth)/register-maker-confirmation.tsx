import { View, Text, Pressable, StyleSheet } from 'react-native'
import { useRouter } from 'expo-router'

export default function RegisterMakerConfirmationScreen() {
  const router = useRouter()

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Inscription réussie !</Text>
      <Text style={styles.subtitle}>
        Bienvenue dans la communauté Passion Market.{'\n'}
        Vous pouvez dès maintenant vous connecter.
      </Text>
      <Pressable style={styles.button} onPress={() => router.push('/')}>
        <Text style={styles.buttonText}>Retour à l'accueil</Text>
      </Pressable>
    </View>
  )
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center', padding: 24 },
  title: { fontSize: 24, fontWeight: 'bold', marginBottom: 16, textAlign: 'center' },
  subtitle: { fontSize: 16, color: '#555', textAlign: 'center', marginBottom: 32 },
  button: {
    backgroundColor: '#2563eb',
    borderRadius: 8,
    paddingVertical: 14,
    paddingHorizontal: 24,
  },
  buttonText: { color: '#fff', fontSize: 16, fontWeight: '600' },
})
