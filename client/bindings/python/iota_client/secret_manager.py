class LedgerNanoSecretManager(dict):
    """Secret manager that uses a Ledger Nano hardware wallet or Speculos simulator.
    """

    def __init__(self, is_simulator):
        """Initialize a ledger nano secret manager.
        """

        dict.__init__(self, ledgerNano=is_simulator)


class MnemonicSecretManager(dict):
    """Secret manager that uses a mnemonic in plain memory. It's not recommended for production use. Use LedgerNano or Stronghold instead.
    """

    def __init__(self, mnemonic):
        """Initialize a mnemonic secret manager.
        """

        dict.__init__(self, mnemonic=mnemonic)


class SeedSecretManager(dict):
    def __init__(self, seed):
        """Initialize a seed secret manager.
        """

        dict.__init__(self, hexSeed=seed)


class StrongholdSecretManager(dict):
    """Secret manager that uses Stronghold.
    """

    def __init__(self, snapshot_path, password):
        """Initialize a stronghold secret manager.
        """

        dict.__init__(self, stronghold=StrongholdSecretManager.Inner(
            snapshot_path, password))

    class Inner(dict):
        def __init__(self, snapshot_path, password):
            dict.__init__(self, password=password, snapshotPath=snapshot_path)
