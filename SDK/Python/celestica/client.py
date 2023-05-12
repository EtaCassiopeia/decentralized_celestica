import grpc
from . import vector_service_pb2
from . import vector_service_pb2_grpc

class CelesticaClient:
    def __init__(self, url):
        self.channel = grpc.insecure_channel(url)
        self.client = vector_service_pb2_grpc.VectorServiceStub(self.channel)

    def insert(self, data, ids):
        float_arrays = [vector_service_pb2.FloatArray(values=d) for d in data]
        request = vector_service_pb2.InsertRequest(data=float_arrays, ids=ids)
        return self.client.Insert(request)

    def search(self, data, knbn, ef):
        float_arrays = [vector_service_pb2.FloatArray(values=d) for d in data]
        request = vector_service_pb2.SearchRequest(data=float_arrays, knbn=knbn, ef=ef)
        response = self.client.Search(request)
        return response.neighbours
