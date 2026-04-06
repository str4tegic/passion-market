import { useState, useEffect } from 'react'
import {
  View,
  Text,
  TextInput,
  Pressable,
  StyleSheet,
  ScrollView,
  KeyboardAvoidingView,
  Platform,
} from 'react-native'
import { useRouter } from 'expo-router'
import { useRegisterMaker } from '@passion-market/hooks'
import { validateRegisterMaker, RegisterMakerErrors } from '@passion-market/api-client'

type Fields = { name: string; email: string; password: string }
type Touched = { name: boolean; email: boolean; password: boolean }

export default function RegisterMakerScreen() {
  const router = useRouter()
  const { status, error, execute } = useRegisterMaker()

  const [values, setValues] = useState<Fields>({ name: '', email: '', password: '' })
  const [touched, setTouched] = useState<Touched>({ name: false, email: false, password: false })
  const [fieldErrors, setFieldErrors] = useState<RegisterMakerErrors>({})

  useEffect(() => {
    if (status === 'success') {
      router.push('/(auth)/register-maker-confirmation')
    }
  }, [status, router])

  function handleChange(field: keyof Fields, value: string) {
    const next = { ...values, [field]: value }
    setValues(next)
    if (touched[field]) {
      setFieldErrors(validateRegisterMaker(next))
    }
  }

  function handleBlur(field: keyof Fields) {
    setTouched(t => ({ ...t, [field]: true }))
    setFieldErrors(validateRegisterMaker(values))
  }

  async function handleSubmit() {
    const allTouched: Touched = { name: true, email: true, password: true }
    setTouched(allTouched)
    const errors = validateRegisterMaker(values)
    setFieldErrors(errors)
    if (Object.keys(errors).length > 0) return
    await execute(values)
  }

  return (
    <KeyboardAvoidingView
      style={{ flex: 1 }}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <ScrollView contentContainerStyle={styles.container}>
        <Text style={styles.title}>Inscription Maker</Text>

        <View style={styles.field}>
          <Text style={styles.label}>Nom</Text>
          <TextInput
            style={[styles.input, touched.name && fieldErrors.name ? styles.inputError : null]}
            value={values.name}
            onChangeText={v => handleChange('name', v)}
            onBlur={() => handleBlur('name')}
            autoCapitalize="words"
          />
          {touched.name && fieldErrors.name && (
            <Text style={styles.errorText}>{fieldErrors.name}</Text>
          )}
        </View>

        <View style={styles.field}>
          <Text style={styles.label}>Email</Text>
          <TextInput
            style={[styles.input, touched.email && fieldErrors.email ? styles.inputError : null]}
            value={values.email}
            onChangeText={v => handleChange('email', v)}
            onBlur={() => handleBlur('email')}
            keyboardType="email-address"
            autoCapitalize="none"
            autoCorrect={false}
          />
          {touched.email && fieldErrors.email && (
            <Text style={styles.errorText}>{fieldErrors.email}</Text>
          )}
        </View>

        <View style={styles.field}>
          <Text style={styles.label}>Mot de passe</Text>
          <TextInput
            style={[
              styles.input,
              touched.password && fieldErrors.password ? styles.inputError : null,
            ]}
            value={values.password}
            onChangeText={v => handleChange('password', v)}
            onBlur={() => handleBlur('password')}
            secureTextEntry
          />
          {touched.password && fieldErrors.password && (
            <Text style={styles.errorText}>{fieldErrors.password}</Text>
          )}
        </View>

        {status === 'error' && error && (
          <Text style={styles.errorText}>
            {error.status === 409
              ? 'Cet email est déjà utilisé.'
              : error.detail || 'Une erreur est survenue.'}
          </Text>
        )}

        <Pressable
          style={[styles.button, status === 'loading' ? styles.buttonDisabled : null]}
          onPress={handleSubmit}
          disabled={status === 'loading'}
        >
          <Text style={styles.buttonText}>
            {status === 'loading' ? 'Inscription en cours…' : "S'inscrire"}
          </Text>
        </Pressable>
      </ScrollView>
    </KeyboardAvoidingView>
  )
}

const styles = StyleSheet.create({
  container: { padding: 24, flexGrow: 1 },
  title: { fontSize: 24, fontWeight: 'bold', marginBottom: 32 },
  field: { marginBottom: 20 },
  label: { fontSize: 14, marginBottom: 6 },
  input: {
    borderWidth: 1,
    borderColor: '#ccc',
    borderRadius: 8,
    paddingHorizontal: 12,
    paddingVertical: 10,
    fontSize: 16,
  },
  inputError: { borderColor: '#e53e3e' },
  errorText: { color: '#e53e3e', fontSize: 12, marginTop: 4 },
  button: {
    backgroundColor: '#2563eb',
    borderRadius: 8,
    paddingVertical: 14,
    alignItems: 'center',
    marginTop: 8,
  },
  buttonDisabled: { opacity: 0.6 },
  buttonText: { color: '#fff', fontSize: 16, fontWeight: '600' },
})
