from iota_client import IotaClient
import os

class IotaBlock:
    """
    A class for handling the block and payload.

    Load data into the block with .load_string() or .load_file()
    Review the block payload with .summary()
    Yield the payload with .payload()
    Submit the payload to the network with .build_block_and_post()

    Once a payload has been posted, the network responce is added.
    """
    # Constructor function
    def __init__(self, tag: str, string: str = None, filepath: str = None, verbose=False):
        self.is_loaded = False
        self.tag = tag
        self.taghex = "0x"+tag.encode("utf-8").hex()

        self.string = string
        self.filepath = filepath
        self.verbose = verbose
        self.block_id = None
        self.block = None
        self.block_url = None

        if string:
            self.load_string(string)
        elif filepath:
            self.load_file(filepath)
        elif self.verbose:
            self.summary()
    

    def summary(self):
        """
        Summarise the payload content
        """
        print(f'BLOCK SUMMARY:')
        print(f'  tag: {self.tag}')
        print(f'  taghex: {self.taghex}')
        print(f'  loaded: {self.is_loaded}')
        if self.is_loaded:
            if self.string:
                print(f'    View original string with .string')
            if self.filepath:
                print(f'    View source filepath with .filepath')
            print(f'    View data (in bytes) with .data')
            print(f'    View data (as a hex) with .datahex')
        else:
            print(f'        Load data with load_string() or load_file()')
        print(f'  block id: {self.block_id}')
        print(f'  block url: {self.block_url}')


    def load_string(self, string: str):
        """
        Load a string as the data payload
        """
        assert not self.is_loaded
        self.loadsource = "string"
        self.string = string
        
        self.data = string.encode("utf-8")
        self.datahex = "0x"+self.data.hex()
        self.is_loaded = True

        if self.verbose:
            self.summary()
    

    def load_file(self, filepath: str):
        """
        Load a file as the data payload
        """
        assert not self.is_loaded
        assert os.path.isfile(filepath)
        self.loadsource = "file"
        self.filepath = filepath

        with open(filepath, 'rb') as f:
            content = f.read()
        self.data = content
        self.datahex = "0x"+self.data.hex()
        self.is_loaded = True

        if self.verbose:
            self.summary()
        
    
    def payload(self):
        """
        Prepare the payload for posting to the shimmer network
        """
        assert self.is_loaded
        return {"tag": self.taghex, "data": self.datahex}
    

    def build_and_post_block(self, client, secret_manager=None):
        """
        Pass block instance to client instance to build and post.
        """
        assert isinstance(client, IotaClient)
        block_id, block = client.build_and_post_block(secret_manager=None, options=self.payload())
        self.block_id = block_id
        self.block = block
        self.block_url = 'https://explorer.iota.org/testnet/block/'+block_id

        if self.verbose:
            self.summary()
        #return block_id, block
        
