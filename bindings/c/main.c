#include <stdint.h>
#include <stdio.h>
#include "include/iota.h"

int main() {
    iota_init("https://nodes.comnet.thetangle.org");
    /*
     * Generate unchecked address
     */
    int8_t seed[243] = {0};
    // Generate the address in index 0.
    int8_t *address = iota_address_gen(seed, 0);
    printf("Generate unchecked address:\n");
    for(int i = 0; i < 243; i++) {
        printf("%d ", address[i]);
    }

    /*
     * Generate new address
     */
    uint8_t err;
    int8_t *new_address = iota_get_new_address(seed, 10, &err);
    printf("\nGenerate new address:\n");
    for(int i = 0; i < 243; i++) {
        printf("%d ", new_address[i]);
    }

    /*
     * Get node info
     */
    printf("\nFollowing node information is retrieved from iota.rs:\n");
    // This is the IRI API call get_nod_info.
    get_node_info_t *node_info = iota_get_node_info(&err);
    // We only define a few fields in the response struct. But this should give a glance how to use it.
    printf("Node name: %s\n", node_info->app_name);
    printf("Node version: %s\n", node_info->app_version);
    printf("Last milestone index: %d\n", node_info->latest_milestone_index);
    printf("err: %d\n", err);
    return 0;
}