using Fermyon.Spin.Sdk;

namespace Sharpie;

public static class Handler
{
    [HttpHandler]
    public static HttpResponse HandleHttpRequest(HttpRequest request)
    {
        return new HttpResponse
        {
            StatusCode = System.Net.HttpStatusCode.OK,
            Headers = new Dictionary<string, string>
            {
                { "Content-Type", "application/json" },
            },
            BodyAsString = "{ \"message\": \"Hello from .NET\" }",
        };
    }
}
