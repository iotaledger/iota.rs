#include <stdint.h>

typedef struct CSeed seed_t;
extern seed_t *iota_seed_new();
extern void iota_seed_free(seed_t *ptr);

typedef struct Address address_t;
extern address_t *iota_address_new();
extern void iota_address_free(address_t *ptr);

typedef struct Hash hash_t;
extern hash_t *iota_hash_new();
extern void iota_hash_free(hash_t *ptr);

typedef struct Transfers transfers_t;
extern transfers_t *iota_transfers_new();
extern void iota_transfers_add(transfers_t *ptr, address_t *address, uint64_t value);
extern void iota_transfers_free(transfers_t *ptr);

typedef struct Bundle bundle_t;
extern bundle_t *iota_bundle_new();
extern void iota_bundle_dbg(bundle_t *ptr);
extern void iota_bundle_free(bundle_t *ptr);

/**
 * @brief Initialize the iota service instance.
 *
 * @param[in] url The node URL
 */
extern void *iota_init(const char *url);

/**
 * @brief The data structure of the get node info response.
 *
 */
typedef struct GetNodeInfoResponse {
  /**
   * Name of IRI node.
   */
  const char *app_name;
  /**
   * Version of IRI node.
   */
  const char *app_version;
  /**
   * Index of the latest milestone.
   */
  uint32_t latest_milestone_index;
} get_node_info_t;

/**
 * @brief Generate IOTA address
 *
 * This is a raw function to generate address which is unchecked by any node.
 * This is just for development purpose and should not be used by users.
 *
 * @param[in] seed A 243 trits long IOTA seed.
 * @param[in] index Index of the address
 * @return A 243 trits long IOTA address.
 */
extern int8_t *iota_address_gen(const int8_t *seed, uint64_t index);


/**
 * @brief Returns information about connected node.
 * 
 * @param[out] error code
 * @return Response type of node information
 */
extern get_node_info_t *iota_get_node_info(uint8_t *err);

/**
 * @brief Generates and returns a new address by calling `find_transactions` until the first unused address is detected.
 * 
 * This stops working after a snapshot.
 * 
 * @param[in] seed A 243 trits long IOTA seed.
 * @param[in] index Index of the address
 * @param[out] address Generated address
 * @return Return status code
 */
extern uint8_t iota_get_new_address(const seed_t *seed, uint64_t index, address_t *address);

/**
 * @brief Calls PrepareTransfers and then sends off the bundle via SendTrytes.
 * 
 * @param[in] seed A 243 trits long IOTA seed.
 * @param[in] transfers Transfer addresses to send data/value to.
 * @param[in] Difficulty of PoW
 * @param[out] bundle The bundle successfully send to tangle
 * @return Return status code
 */
extern uint8_t iota_send_transfers(const seed_t *seed, transfers_t *transfers, uint8_t mwm, bundle_t *bundle);

/* @brief Fetches and validates the bundle given a tail transaction hash, and traversing through trunk transaction.
 * 
 * @param[in] hash Tail transaction hash (current_index == 0)
 * @param[out] bundle The bundle successfully send to tangle
 * @return Return status code
 */
extern uint8_t iota_traverse_bundle(const hash_t *hash, bundle_t *bundle);