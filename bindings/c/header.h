#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Address;

struct CSeed;

template<typename T>
struct Vec;

struct GetNodeInfoResponse {
  const char *app_name;
  const char *app_version;
  uint32_t latest_milestone_index;
};

struct CTransfers {
  Vec<Transfer> inner;
};

extern "C" {

void iota_address_free(Address *ptr);

const int8_t *iota_address_gen(const int8_t *seed, uint64_t index);

const Address *iota_get_new_address(const CSeed *seed, uint64_t index, uint8_t *err);

GetNodeInfoResponse *iota_get_node_info(uint8_t *err);

void iota_init(const char *url);

void iota_seed_free(CSeed *ptr);

CSeed *iota_seed_new();

void iota_send_transfers(const CSeed *seed, CTransfers *transfers, uint8_t mwm, uint8_t *err);

} // extern "C"
