export function PageShimmer() {
  return (
    <div className="p-4">
      <div className="flex items-end justify-between gap-3">
        <div className="space-y-2">
          <div className="shimmer h-5 w-40" />
          <div className="shimmer h-3 w-64" />
        </div>
        <div className="shimmer h-8 w-32" />
      </div>

      <div className="mt-4 grid gap-4 lg:grid-cols-2">
        <div className="panel p-3">
          <div className="shimmer mb-4 h-3 w-28" />
          <div className="space-y-2">
            {[0, 1, 2, 3].map((i) => (
              <div key={i} className="shimmer h-10 w-full" />
            ))}
          </div>
        </div>
        <div className="panel p-3">
          <div className="shimmer mb-4 h-3 w-28" />
          <div className="space-y-2">
            {[0, 1, 2, 3].map((i) => (
              <div key={i} className="shimmer h-10 w-full" />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}