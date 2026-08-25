import SwiftUI

public struct UnlockVaultView: View {
    @ObservedObject var repository = VaultRepository.shared
    @State private var enteredPin: String = ""
    @State private var errorMessage: String? = nil
    @State private var isShaking: Bool = false

    private let generator = UIImpactFeedbackGenerator(style: .medium)

    public init() {}

    public var body: some View {
        ZStack {
            Color(red: 0.06, green: 0.09, blue: 0.16)
                .ignoresSafeArea()

            VStack(spacing: 28) {
                Spacer()

                // Brand Hero
                VStack(spacing: 12) {
                    ZStack {
                        Circle()
                            .fill(
                                LinearGradient(
                                    colors: [Color.indigo, Color.purple],
                                    startPoint: .topLeading,
                                    endPoint: .bottomTrailing
                                )
                            )
                            .frame(width: 80, height: 80)
                            .shadow(color: Color.indigo.opacity(0.4), radius: 16, x: 0, y: 8)

                        Image(systemName: "shield.checkered")
                            .font(.system(size: 38, weight: .bold))
                            .foregroundColor(.white)
                    }

                    Text("Orvpass")
                        .font(.system(size: 26, weight: .bold, design: .rounded))
                        .foregroundColor(.white)

                    Text("Touch ID / Face ID or Quick PIN")
                        .font(.system(size: 13, weight: .medium))
                        .foregroundColor(.gray)
                }

                // 4-Dot PIN Indicator
                HStack(spacing: 18) {
                    ForEach(0..<4, id: \.self) { index in
                        Circle()
                            .fill(index < enteredPin.count ? Color.indigo : Color.white.opacity(0.15))
                            .frame(width: 14, height: 14)
                            .overlay(
                                Circle()
                                    .stroke(Color.indigo.opacity(index < enteredPin.count ? 0.8 : 0.0), lineWidth: 2)
                            )
                            .scaleEffect(index < enteredPin.count ? 1.2 : 1.0)
                            .animation(.spring(response: 0.25, dampingFraction: 0.6), value: enteredPin.count)
                    }
                }
                .padding(.vertical, 8)
                .offset(x: isShaking ? -10 : 0)

                if let error = errorMessage {
                    Text(error)
                        .font(.system(size: 12, weight: .semibold))
                        .foregroundColor(.red)
                        .transition(.opacity)
                }

                // Biometric Hero Unlock Button
                Button(action: triggerBiometrics) {
                    HStack(spacing: 8) {
                        Image(systemName: "faceid")
                            .font(.system(size: 18, weight: .semibold))
                        Text("Unlock with Face ID")
                            .font(.system(size: 14, weight: .semibold))
                    }
                    .padding(.horizontal, 20)
                    .padding(.vertical, 12)
                    .background(Color.white.opacity(0.08))
                    .foregroundColor(.white)
                    .clipShape(Capsule())
                }

                // 3x4 Numeric Keypad
                VStack(spacing: 16) {
                    ForEach(0..<3) { row in
                        HStack(spacing: 24) {
                            ForEach(1..<4) { col in
                                let digit = "\(row * 3 + col)"
                                KeypadButton(label: digit) {
                                    appendDigit(digit)
                                }
                            }
                        }
                    }
                    HStack(spacing: 24) {
                        Button(action: triggerBiometrics) {
                            Image(systemName: "touchid")
                                .font(.system(size: 22))
                                .foregroundColor(.indigo)
                                .frame(width: 72, height: 72)
                                .background(Color.white.opacity(0.05))
                                .clipShape(Circle())
                        }

                        KeypadButton(label: "0") {
                            appendDigit("0")
                        }

                        Button(action: deleteDigit) {
                            Image(systemName: "delete.left.fill")
                                .font(.system(size: 20))
                                .foregroundColor(.gray)
                                .frame(width: 72, height: 72)
                                .background(Color.white.opacity(0.05))
                                .clipShape(Circle())
                        }
                    }
                }

                Spacer()
            }
            .padding(.horizontal, 32)
        }
        .onAppear {
            triggerBiometrics()
        }
    }

    private func appendDigit(_ digit: String) {
        guard enteredPin.count < 4 else { return }
        generator.impactOccurred()
        enteredPin.append(digit)
        errorMessage = nil

        if enteredPin.count == 4 {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
                let success = repository.unlockWithPin(enteredPin)
                if !success {
                    withAnimation(.default) {
                        isShaking = true
                        errorMessage = "Incorrect PIN. Try again."
                    }
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                        isShaking = false
                        enteredPin = ""
                    }
                }
            }
        }
    }

    private func deleteDigit() {
        guard !enteredPin.isEmpty else { return }
        generator.impactOccurred()
        enteredPin.removeLast()
        errorMessage = nil
    }

    private func triggerBiometrics() {
        repository.authenticateWithBiometrics { success in
            if !success {
                // Keep PIN keypad ready
            }
        }
    }
}

struct KeypadButton: View {
    let label: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(label)
                .font(.system(size: 26, weight: .semibold, design: .rounded))
                .foregroundColor(.white)
                .frame(width: 72, height: 72)
                .background(Color.white.opacity(0.08))
                .clipShape(Circle())
        }
    }
}
