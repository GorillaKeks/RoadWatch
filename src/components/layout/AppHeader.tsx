interface AppHeaderProps {
  memberCount: number;
  onlineCount: number;
}

export function AppHeader({
  memberCount,
  onlineCount,
}: AppHeaderProps) {
  return (
    <header className="app-header">
      <h1>ROADWATCH</h1>

      <div className="statistics">
        <span>
          VTC Members:{" "}
          <strong>{memberCount}</strong>
        </span>

        <span className="separator">
          •
        </span>

        <span>
          Online:{" "}
          <strong className="online-text">
            {onlineCount}
          </strong>
        </span>
      </div>
    </header>
  );
}
