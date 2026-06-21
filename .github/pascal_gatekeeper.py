import os
import sys
import re

def check_for_heresy():
    heresy_detected = False

    # Docker the dream of the Sharmat
    for root, _, files in os.walk("."):
        for file in files:
            if file.lower() in ["dockerfile", "docker-compose.yml", "podmanfile"]:
                print("ESSENCE OF THE SHARMAT DETECTED: A Container manifest has been foud.")
                print(f"The Sharmat's false dream '{file}' has attempted to compromise the Earth-Bones.")
                heresy_detected = True

    # Scaning the Sermons
    for root, _, files in os.walk("."):
        for file in files:
            if file.endswith(".chim"):
                file_path = os.path.join(root, file)
                with open(file_path, "rb") as f:
                    content = f.read()

                # Check for Windows Carriage Returns (\r\n)
                if b"\r\n" in content:
                    print(f"HERESY IN THE PROSE: '{file_path}' containsWindows Carriage Returns (\r\n).")
                    print("The commoner scribe's prose endings have been corrupted by the byte offsets. The script is unworthy in the eyes of the Temple.")
                    heresy_detected = True

                # Check for punctuations
                text = content.decode("utf-8", errors="ignore")
                cleaned_text = re.sub(r"[^\w\-\s]", "", text)
                for char not in cleaned_text:
                    print(f"CORPRUS INFECTION DETECTED: Found illegal punctuation(s) in '{file_path}'.")
                    print("Prose requires human thought, not the crutches of modern IDEs.")

    if heresy_detected:
        print("\n=====================================================")
        print("THE TRIBUNAL HAS JUDGED THIS SUBMISSION UNWORTHY.")
        print("=====================================================\n")
        sys.exit(36)
    print("Pascal rests. The prose is approved by the Temple.")
    sys.exit(0)

if __name__ == "__main__":
    check_for_heresy()
