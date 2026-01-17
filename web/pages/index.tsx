import Head from 'next/head';

export default function Home() {
  return (
    <>
      <Head>
        <title>SoroScope Dashboard</title>
        <meta name="description" content="Soroban resource profiler dashboard" />
      </Head>
      <main className="min-h-screen bg-slate-950 text-slate-100 flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-4xl font-bold mb-4">SoroScope</h1>
          <p className="text-slate-300">Soroban Resource Profiler – Web Dashboard</p>
        </div>
      </main>
    </>
  );
}
