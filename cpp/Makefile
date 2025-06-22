# Kaynak dosyanızın adı
SOURCE = main.cpp
# Çıktı çalıştırılabilir dosyanın adı
TARGET_NAME = myprogram
# Çıktının yer alacağı dizin
BUILD_DIR = build

# Tam çıktı yolu (örneğin build/myprogram)
TARGET = $(BUILD_DIR)/$(TARGET_NAME)

# Varsayılan hedef: build dizinini oluştur, derle ve çalıştır
all: $(BUILD_DIR) $(TARGET)
	./$(TARGET)

# build dizinini oluşturma kuralı (eğer yoksa)
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Programı derleme kuralı
$(TARGET): $(SOURCE)
	g++ $(SOURCE) -o $(TARGET)

# Temizlik (derlenmiş dosyayı ve build dizinini siler)
clean:
	rm -f $(TARGET)
	rmdir $(BUILD_DIR) 2>/dev/null || true
