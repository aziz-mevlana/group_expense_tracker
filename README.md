# Soroban Group Expense Tracker (Grup Gider Takipçisi)

Bu proje, **Soroban SDK** ve **Rust** kullanılarak Stellar Blockchain üzerinde geliştirilmiş merkeziyetsiz bir gider paylaşım uygulamasıdır. 

"Splitwise" benzeri bir mantıkla çalışır; grup üyelerinin ortak harcamalarını kaydeder ve kimin kime ne kadar borcu olduğunu otomatik olarak hesaplar.

## 📂 Proje Yapısı (Project Structure)

Bu repo, Soroban projeleri için önerilen standart yapıyı kullanır:

````
.
├── contracts
│   └── group_expense_tracker
│       ├── src
│       │   ├── lib.rs      # Sözleşmenin ana mantığı ve fonksiyonları
│       │   └── test.rs     # Birim testleri (Unit tests)
│       └── Cargo.toml      # Sözleşme bağımlılıkları
├── Cargo.toml              # Workspace bağımlılıkları
└── README.md               # Proje dokümantasyonu
````

## ⚙️ Teknik Detaylar ve Fonksiyonlar

### 1\. Veri Yapıları

Veriler blockchain üzerinde `Storage` (Depolama) kullanılarak saklanır.

  * **`struct Expense`**: Bir harcamanın detaylarını tutar (Ödeyen kişi, Miktar, Açıklama).
  * **`enum DataKey`**: Verilere erişmek için kullanılan anahtarlardır (`Members` ve `Expenses`).

### 2\. Akıllı Sözleşme Fonksiyonları

#### `initialize(env: Env, members: Vec<Address>)`

Sözleşme ağa yüklendiğinde grubu kurmak için **sadece bir kez** çalıştırılır.

  * **İşlevi:** Belirtilen cüzdan adreslerini "Grup Üyesi" olarak kaydeder ve harcamalar için boş bir liste oluşturur.
  * **Güvenlik:** Eğer sözleşme daha önce başlatıldıysa "Contract already initialized" hatası verir.

#### `add_expense(env: Env, payer: Address, amount: u128, description: String)`

Gruba yeni bir harcama ekler.

  * **Yetkilendirme:** `payer.require_auth()` fonksiyonu ile işlemi yapan kişinin dijital imzasını zorunlu kılar.
  * **Üye Kontrolü:** Sadece grupta kayıtlı olan üyeler harcama ekleyebilir.
  * **Kayıt:** Yeni harcamayı oluşturur ve blockchain hafızasındaki (Storage) listeye ekler.

#### `get_balances(env: Env) -> Map<Address, i128>`

Grubun güncel borç durumunu hesaplar.

  * **Mantık:**
    1.  Toplam harcamayı ve kişi başına düşen payı hesaplar.
    2.  Her üye için `Ödenen - Pay` formülünü uygular.
  * **Sonuç:**
      * **Pozitif (+)** sonuç: Kişi gruptan alacaklıdır.
      * **Negatif (-)** sonuç: Kişi gruba borçludur.

## 🚀 Kurulum ve Test

Bu projeyi yerel ortamınızda çalıştırmak için aşağıdaki adımları izleyebilirsiniz.

### Ön Gereksinimler

  * Rust ve Cargo
  * Soroban CLI

### 1\. Testleri Çalıştırın

Sözleşmenin mantığını doğrulamak için yazılmış birim testlerini çalıştırın:

```bash
cargo test
```

### 2\. Derleme (Build)

Projeyi Stellar ağına yüklenebilir `.wasm` formatına dönüştürün:

```bash
stellar contract build
```

### 3\. Dağıtım (Deploy - Testnet)

Sözleşmeyi Stellar Test Ağına yükleyin:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/group_expense_tracker.wasm \
  --source [HESAP_ADINIZ] \
  --network testnet
```

## 🧪 Örnek Kullanım (CLI)

Terminal üzerinden sözleşme ile etkileşime geçmek için örnek komutlar:

```bash
# 1. Grubu Kur (Örnek Adresler)
stellar contract invoke --id [CONTRACT_ID] --source alice --network testnet -- initialize --members '["ALICE_ADDRESS", "BOB_ADDRESS"]'

# 2. Harcama Ekle
stellar contract invoke --id [CONTRACT_ID] --source alice --network testnet -- add_expense --payer ALICE_ADDRESS --amount 100 --description "Market"

# 3. Bakiyeleri Gör
stellar contract invoke --id [CONTRACT_ID] --source alice --network testnet -- get_balances
```
