import { Stack } from 'expo-router'

export default function AuthLayout() {
  return (
    <Stack>
      <Stack.Screen name="register-maker" options={{ title: 'Inscription Maker' }} />
      <Stack.Screen
        name="register-maker-confirmation"
        options={{ title: 'Confirmation', headerLeft: () => null }}
      />
    </Stack>
  )
}
